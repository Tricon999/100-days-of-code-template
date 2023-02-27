# Quick Start

## Profile fzy implementation in Pure Python and Rust

```bash
$ cd bench/python
```

```bash
$ cd test
$ bash fetch_testdata.sh
$ ./run-profile.sh --all
```

- OS: macOS 10.14.6
- Machine: MBP 18 15-inch, 2.2GHz Intel Core i7, 32 GB 2400 MHz DDR4.

<table style="width: 100%;">

<tr><th>Pure Python</th><th>Rust</th></tr>
<tr>

<td>

```
stats of pure Python fuzzy filter performance:

total items: 100257
[once]
====== vim ======
FUNCTION  <SNR>37_ext_filter()
1   5.951908             <SNR>37_ext_filter()

====== nvim =