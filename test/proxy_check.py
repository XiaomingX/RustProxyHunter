import requests
from concurrent.futures import ThreadPoolExecutor

## 批量验证的代理服务器.

# 代理列表
proxies_list = [
    {"ip": "47.93.0.20", "port": "8080"},
    {"ip": "47.93.0.23", "port": "8081"},
    {"ip": "47.93.0.24", "port": "8080"},
    {"ip": "47.93.0.30", "port": "9999"},
    {"ip": "47.93.0.35", "port": "8888"},
    {"ip": "47.93.0.56", "port": "8888"},
    {"ip": "47.93.0.61", "port": "9999"},
    {"ip": "47.93.0.82", "port": "8888"},
    {"ip": "47.93.0.102", "port": "8888"},
    {"ip": "47.93.0.115", "port": "8080"},
    {"ip": "47.93.0.129", "port": "8000"},
    {"ip": "47.93.0.179", "port": "8888"},
    {"ip": "47.93.0.212", "port": "8081"},
    {"ip": "47.93.0.224", "port": "8888"},
]

def check_proxy(proxy):
    proxies = {
        "http": f"http://{proxy['ip']}:{proxy['port']}",
        "https": f"http://{proxy['ip']}:{proxy['port']}"
    }
    try:
        response = requests.get("https://www.baidu.com/", proxies=proxies, timeout=5)
        response.raise_for_status()
        print(f"代理 {proxy['ip']}:{proxy['port']} 可用")
        return True
    except requests.exceptions.RequestException:
        print(f"代理 {proxy['ip']}:{proxy['port']} 不可用")
        return False

# 使用线程池进行批量验证
with ThreadPoolExecutor(max_workers=10) as executor:
    executor.map(check_proxy, proxies_list)
