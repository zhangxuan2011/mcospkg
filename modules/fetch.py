"""
=====Description=====
File in 'modules/fetch.py'
To download files(espectially package) files from remote server
"""

import http.client


def common_fetch(url,filename):
	# Explain URL
	parsed_url = http.client.urlsplit(url)
	conn = http.client.HTTPSConnection(parsed_url.netloc)
	
	# Send requests
	conn.request('GET', parsed_url.path)
	response = conn.getresponse()
	
	# Save file
	with open(filename, 'wb') as file:
		file.write(response.read())
	return 0
