# webmonito-rs

**Monitors websites and sends emails and web requests when they change**

## Features

* Monitor changes for selected websites
* Set a custom timeout between checks
* Send customisable emails or ping urls when a change is detected

The program is also very lightweight and easy to use.

## Usage

To get started and see all available options, just use the `-h` flag.

```bash
$ ./wbmrs -h
```

There are two ways to use this program: with command line options or with a config file.  
If you use both, the command line options will override or add up to the ones specified in the config file.

### CLI usage

You need to provide two required arguments:

* `-u` or `--urls` which takes a list of urls to monitor.
* `-t` or `--timeout` which takes a timeout in whole minutes between checks.

For instance, the command  
```bash
$ ./wbmrs -u https://www.rust-lang.org/ https://docs.rs/ -t 60
```  
would monitor https://www.rust-lang.org/ and https://docs.rs/ for changes every 60 minutes and output to the console if a change is detected.

### Config file usage

Config files are in `toml` format. You can specify a config file like this:  
```bash
$ ./wbmrs config.toml
```

Here is how a config file should look:  
```toml
# Specify verbose (optional)
verbose = true

# Specify timeout (required)
timeout = 60

# Specify urls to monitor (required)
urls = [
    "https://google.com",
    "https://rust-lang.org",
]

# Specify the adress to use to send emails (optional)
sender = "wbmrs@raphaeltheriault.com"

# Use [[emails]] to specify an email to send on change (optional)
[[emails]]
# Adress of the recipient (required)
address = "raphael_theriault@outlook.com"
# Content of the email (optional)
content = "Hello, World!"

[[emails]]
address = "test@example.com"

# Use [[pings]] to specify a ping to send on change (optional)
[[pings]]
# Url to ping (required)
url = "https://api.github.com"
# Content of the ping (optionnal)
content = "ping"

[[pings]]
url = "https://example.com"
```

## Details

### Emails

Emails are sent with the following contents:  
```text
Webpage at <URL> has been updated.
<CONTENT>
```

They are sent using the `sendmail` command.

You should specify a sender corresponding to your current IP, as most email services will consider the emails spam if it doesn't correspond, or even block the emails.

### Pings

Pings are sent as `POST` requests with `JSON` content with the following format:
```json
{
  "url": "<URL>",
  "message": "<CONTENT>"
}
```

If you enable verbose, responses will be shown in the output.

## License

This program is dual licensed under the [MIT license](LICENSE-MIT) or the [Apache License, Version 2.0](LICENSE-APACHE).