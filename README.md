# mf\_attendance

Moneyforward Cloud Attendance client written in Rust.

## Install

```
$ git clone https://github.com/alpaca-tc/mf_attendance-rs
$ cd mf_attendance-rs
$ cargo build
$ cargo install
```

### Usage

```
# Login
$ mf_attendance login
Copy your api token from https://attendance.moneyforward.com/my_page/settings/employee_api_token
Paste your api token: ...

# Create record
$ mf_attendance clock_in
$ mf_attendance clock_out
$ mf_attendance start_break
$ mf_attendance end_break
```
