
There is no easy way like Elastic Beanstalk or Lambda (which does not yet support actix-web framework) to deploy a Rust webapp with Actix Web framework, we need to do everything from stcratch using EC2.

- Step 1: Launch a EC2 instance (I choosed Ubuntu 22 LTS)

- Step 2: Update EC2 instance and install git, rust (and may be openssl , g++/gcc, depends on the OS you choosed for EC2 instance)

- Step 3: Look for security tab from the EC2 instance detail page, click the security group link to go to scurity group detail page. Then click `Edit inbound rules` -> `Add rules` , then select (1): `Custome TCP` (Type) + `3000` (Port range) + `0.0.0.0/0` (CIDR block) ; (2): `Custome UDP` (Type) + `2123` (Port range) + `0.0.0.0/0` (CIDR block); Then hit `Save rules` button to add the above 2 inbound rules.

- Step 4: Connect to EC2 virtual machine, generate SSH key and pull the Rust server code from github to your EC2.

- Step 5: Create a `.host` file in the root of the project. Look for **Private IPv4 addresses** of your instance from the instance detail page and put it to the `.host` file. This will replace the localhots / 127.0.0.1 in the code when running on your local machine

- Step 6: run the Rust web server by run `cargo build && cargo run`, make sure it is running.

- Step 7: Now look for **Public IPv4 address** from the EC2 intsance detail page, which should be right beside the **Private IPv4 addresses**, mine is **34.220.237.129**. That is the host you will use to visit your rust server. Try `curl -v http://[Public IPv4 address]:3000/api/v2/tags` you shoud see the response as following

```
➜  rust_blog_service git:(sample) curl -v http://34.220.237.129:3000/api/v2/tags  
*   Trying 34.220.237.129:3000...
* Connected to 34.220.237.129 (34.220.237.129) port 3000 (#0)
> GET /api/v2/tags HTTP/1.1
> Host: 34.220.237.129:3000
> User-Agent: curl/7.81.0
> Accept: */*
> 
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< content-length: 116
< content-type: application/json
< date: Sat, 15 Apr 2023 20:47:53 GMT
< 
* Connection #0 to host 34.220.237.129 left intact
[{"id":1,"tagname":"sport","description":null},{"id":2,"tagname":"economics","description":"Topics about economic"}]%     
```

The dummy data responsed as JSON from the server

- Step 8. We need to make it as a system service so that it can keep running event we exit the terminal. To do that, create 
