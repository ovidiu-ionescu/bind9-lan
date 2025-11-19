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

; Extra config
domain                home.lab
dns_file_name         db.lab
reverse_dns_file_name db.0.0.10
dhcp_file_name        reservations.conf

; Prefix and suffix for the db.lab file
DNS_PREFIX """
; Generated from netconfig.conf
$TTL    604800
@       IN      SOA     home.lab. admin.home.lab. (
                              2         ; Serial
                         604800         ; Refresh
                          86400         ; Retry
                        2419200         ; Expire
                         604800 )       ; Negative Cache TTL
;
@       IN      NS      home.lab.
"""

DNS_SUFFIX """
@ 3600 IN MX 3 ALT1.ASPMX.L.GOOGLE.COM.
@ 3600 IN MX 3 ALT2.ASPMX.L.GOOGLE.COM.
@ 3600 IN MX 1 ASPMX.L.GOOGLE.COM.
@ 3600 IN MX 5 ASPMX2.GOOGLEMAIL.COM.
@ 3600 IN MX 5 ASPMX3.GOOGLEMAIL.COM.
@ 3600 IN MX 5 ASPMX4.GOOGLEMAIL.COM.
@ 3600 IN MX 5 ASPMX5.GOOGLEMAIL.COM.
"""

DHCP_PREFIX """
# Generated from netconfig.conf
"""
```
