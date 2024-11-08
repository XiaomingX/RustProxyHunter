import os

def main():
    subnets = {
        "香港": [
            "47.56.0.0/15",
            "47.244.0.0/16",
            "47.75.0.0/16"
        ],
        "新加坡": [
            "47.74.128.0/17",
            "47.88.192.0/18"
        ],
        "日本": [
            "47.74.0.0/18",
            "47.245.0.0/18"
        ],
        "美国": [
            "47.251.0.0/16",
            "47.254.0.0/17"
        ],
        "德国": [
            "47.254.128.0/18"
        ],
        "马来西亚": [
            "47.254.192.0/18",
            "47.250.0.0/16"
        ]
    }

    for region, ips in subnets.items():
        for ip in ips:
            command = f"./target/release/port_scanner -ips {ip}"
            print(f"Executing command for {region}: {command}")
            os.system(command)
            print()
            print()

if __name__ == "__main__":
    main()
