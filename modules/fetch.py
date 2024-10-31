"""
=====Description=====
File in 'modules/fetch.py'
To download files(espectially package) files from remote server
"""

import http.client
import requests
from tqdm import tqdm


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


def download_with_pgb(url, output_path):
    response = requests.get(url, stream=True)
    total_size = int(response.headers.get('content-length', 0))
    block_size = 1024  # 1 Kibibyte

    with open(output_path, 'wb') as file, tqdm(
        desc=output_path,
        total=total_size,
        unit='iB',
        unit_scale=True,
        unit_divisor=1024,
    ) as bar:
        for data in response.iter_content(block_size):
            bar.update(len(data))
            file.write(data)
