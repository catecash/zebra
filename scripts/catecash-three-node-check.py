#!/usr/bin/env python3
"""Compare the three hosted Catecash devnet nodes without exposing RPC cookies."""

import base64
import json
import time
from pathlib import Path
from urllib.request import Request, urlopen


GENESIS = "6b9c4d783359202111571c123309b2c0511d1bf9036f74dee4921f04e1f0a9cf"
MINER_ADDRESS = "yPcjsh1MxoRFtkaye1hjrZqi2ZGsB7jby1H"
SUBSIDY = 500_000_000_000
NODES = {
    "miner": (34342, "/var/lib/catecash/rpc/.cookie"),
    "peer": (34345, "/var/lib/catecash/peer-rpc/.cookie"),
    "third": (34348, "/var/lib/catecash/third-rpc/.cookie"),
}


def rpc(node, method, params=None):
    port, cookie_file = NODES[node]
    cookie = Path(cookie_file).read_text().strip()
    authorization = base64.b64encode(cookie.encode()).decode()
    request = Request(
        f"http://127.0.0.1:{port}/",
        data=json.dumps({"jsonrpc": "2.0", "id": 1, "method": method,
                         "params": params or []}).encode(),
        headers={"Content-Type": "application/json",
                 "Authorization": "Basic " + authorization},
    )
    with urlopen(request, timeout=15) as response:
        body = json.load(response)
    if body.get("error"):
        raise RuntimeError(body["error"])
    return body["result"]


def compare():
    info = {name: rpc(name, "getblockchaininfo") for name in NODES}
    height = min(item["blocks"] for item in info.values())
    common_hashes = {name: rpc(name, "getblockhash", [height])
                     for name in NODES}
    result = {}
    for name, item in info.items():
        balance = rpc(name, "getaddressbalance",
                      [{"addresses": [MINER_ADDRESS]}])
        pools = {pool["id"]: int(pool["chainValueZat"])
                 for pool in item["valuePools"]}
        result[name] = {
            "chain": item["chain"],
            "height": item["blocks"],
            "tip_hash": item["bestblockhash"],
            "common_hash": common_hashes[name],
            "pool_total": str(sum(pools.values())),
            "expected_issued": str(item["blocks"] * SUBSIDY),
            "miner_balance": str(balance["balance"]),
            "miner_received": str(balance["received"]),
            "peer_count": len(rpc(name, "getpeerinfo")),
        }
    return height, result


def main():
    for name in NODES:
        if rpc(name, "getblockhash", [0]).lower() != GENESIS:
            raise RuntimeError(f"{name} has wrong genesis")
    deadline = time.monotonic() + 180
    while True:
        height, result = compare()
        heights = [item["height"] for item in result.values()]
        hashes = {item["common_hash"] for item in result.values()}
        ready = (
            height >= 1
            and max(heights) - height <= 5
            and len(hashes) == 1
            and all(item["chain"] == "catecash-devnet" for item in result.values())
            and all(item["pool_total"] == item["expected_issued"]
                    for item in result.values())
            and all(int(item["miner_received"]) >= int(item["expected_issued"])
                    for item in result.values())
            and all(item["peer_count"] >= 1 for item in result.values())
        )
        if ready or time.monotonic() >= deadline:
            print(json.dumps({"ready": ready, "common_height": height,
                              "nodes": result}, sort_keys=True))
            return 0 if ready else 1
        time.sleep(5)


if __name__ == "__main__":
    raise SystemExit(main())
