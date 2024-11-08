import os
from datetime import datetime

def main():
    # 获取当前日期并创建文件夹
    today = datetime.today().strftime('%Y-%m-%d')
    folder_path = os.path.join(os.getcwd(), today)
    os.makedirs(folder_path, exist_ok=True)

    # 在文件夹中创建README.md文件
    readme_path = os.path.join(folder_path, "README.md")
    with open(readme_path, "w") as readme_file:
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
                result = os.popen(command).read()  # 执行命令并获取结果
                # 写入执行命令的区域、IP和结果到README.md
                readme_file.write(f"## {region}\n")
                readme_file.write(f"### IP: {ip}\n")
                readme_file.write("```\n")
                readme_file.write(result)
                readme_file.write("```\n\n")
                print(f"Executed command for {region}: {command}")

if __name__ == "__main__":
    main()
