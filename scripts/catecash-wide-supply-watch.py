#!/usr/bin/env python3
"""Record Catecash devnet subsidy-boundary snapshots from a local Zebra RPC."""

import base64
import json
import sys
import time
from pathlib import Path
from urllib.request import Request, urlopen


RPC_URL = "http://127.0.0.1:34342/"
COOKIE_FILE = Path("/var/lib/catecash/rpc/.cookie")
OUTPUT_FILE = Path("/var/lib/catecash/wide-supply-run.jsonl")
GENESIS = "6b9c4d783359202111571c123309b2c0511d1bf9036f74dee4921f04e1f0a9cf"
MINER_ADDRESS = "yPcjsh1MxoRFtkaye1hjrZqi2ZGsB7jby1H"
SUBSIDY = 500_000_000_000
MILESTONES = (4200, 4201)


def rpc(method, params=None):
    cookie = COOKIE_FILE.read_text().strip()
    authorization = base64.b64encode(cookie.encode()).decode()
    request = Request(
        RPC_URL,
        data=json.dumps({"jsonrpc": "2.0", "id": 1, "method": method,
                         "params": params or []}).encode(),
        headers={"Content-Type": "application/json",
                 "Authorization": "Basic " + authorization},
    )
    with urlopen(request, timeout=15) as response:
        result = json.load(response)
    if result.get("error"):
        raise RuntimeError(result["error"])
    return result["result"]


def snapshot():
    info = rpc("getblockchaininfo")
    height = info["blocks"]
    address = rpc("getaddressbalance", [{"addresses": [MINER_ADDRESS]}])
    if rpc("getblockchaininfo")["blocks"] != height:
        return None
    pools = {pool["id"]: int(pool["chainValueZat"])
             for pool in info["valuePools"]}
    circulating = sum(pools.values())
    expected = height * SUBSIDY
    return {
        "height": height,
        "hash": info["bestblockhash"],
        "pools": {name: str(value) for name, value in pools.items()},
        "circulating": str(circulating),
        "expected_issued": str(expected),
        "supply_matches": circulating == expected,
        "miner_address": MINER_ADDRESS,
        "miner_balance": str(address["balance"]),
        "miner_received": str(address["received"]),
    }


def record(value):
    line = json.dumps(value, sort_keys=True)
    with OUTPUT_FILE.open("a") as output:
        output.write(line + "\n")
    print(line, flush=True)


def main():
    if rpc("getblockhash", [0]).lower() != GENESIS:
        raise RuntimeError("wrong Catecash devnet genesis")
    if sys.argv[1:] == ["--once"]:
        print(json.dumps(snapshot(), sort_keys=True))
        return 0
    captured = set()
    while True:
        try:
            value = snapshot()
            if value is None:
                time.sleep(1)
                continue
            height = value["height"]
            if height >= 4200 and not value["supply_matches"]:
                record({"error": "issued supply mismatch", **value})
                raise RuntimeError("issued supply does not match subsidy schedule")
            for milestone in MILESTONES:
                if height == milestone and milestone not in captured:
                    record(value)
                    captured.add(milestone)
            if height >= MILESTONES[-1]:
                if MILESTONES[-1] not in captured:
                    record({"note": "first observed tip above boundary", **value})
                return 0
            time.sleep(30 if height < 4180 else 1)
        except (OSError, TimeoutError, KeyError, ValueError) as error:
            print(f"RPC snapshot unavailable: {error}", file=sys.stderr, flush=True)
            time.sleep(10)


if __name__ == "__main__":
    raise SystemExit(main())
