import http.client


def common_fetch(url,filename):
	# 解析URL
	parsed_url = http.client.urlsplit(url)
	conn = http.client.HTTPSConnection(parsed_url.netloc)
	
	# 发送请求
	conn.request('GET', parsed_url.path)
	response = conn.getresponse()
	
	# 保存文件
	with open(filename, 'wb') as file:
		file.write(response.read())
	return 0
