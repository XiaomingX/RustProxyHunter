## 用来验证是否成功。

import requests

proxies = {
    "http": "http://108.61.201.18:8888",
}
headers = {
    "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
}

try:
    # 发起GET请求，带上代理和header
    response = requests.get("https://www.baidu.com/", proxies=proxies, headers=headers, timeout=10)
    response.raise_for_status()  # 检查请求是否成功
    content = response.text  # 获取网页内容（字符串形式）
    print(content)
except requests.exceptions.RequestException as e:
    print(f"请求失败: {e}")
