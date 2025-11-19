# tree-sitter-orgncf

A Tree-sitter grammar for the Org NCf (Network Configuration Format) language.

If you want to assign fixed ip addesses to devices in your network and give them
DNS names, you normally need to maintain three separate configuration files:

- the DHCP server config file that describes what ip address to assign to which
  device MAC address,
- the DNS server config file that maps DNS names to ip addresses, and
- the reverse DNS server config file that maps ip addresses back to DNS names.

A single configuration file can be used to generate all these three files.
This is what Org NCf is for.

Example:

```orgncf
; comment
71:e7:22:ae:a5:f2 10.0.0.10 printer ; my network printer
ad:55:4e:db:50:be 10.0.0.11 tv ; my smart tv
```
