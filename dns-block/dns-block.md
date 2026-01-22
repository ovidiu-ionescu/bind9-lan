---
title: dns-block
section: 1
header: User Manual
footer: bind9-lan
date: January 2026
---

# NAME
dns-block - process dns block lists

# SYNOPSIS
Usage: dns-block [OPTIONS] <COMMAND>

# DESCRIPTION
Simplify the list of ad and tracking servers to block for DNS based blocking in bind9.
Fetch block lists from multiple sources, merge them, remove duplicates and optionally
allow some domains to pass through even if they are in the block lists. The is done with
a whitelisting file.


# COMMANDS:
*pack* 
: Pack the domains list into one file

*pipe*
: Act as a pipe when tailing the Bind9 query log
  
*help*
: Print this message or the help of the given subcommand(s)

# OPTIONS:
*-d, --debug...*                    
: log level, dddd for trace, ddd for debug, dd for info, d for warn, default no output
  
*-t, --timing*                      
: display timing information at the end of processing

*-l, --lists-file <LISTS_FILE>...*
: File containing a list of URLs to fetch block lists from, multiple can be specified

*-b, --block-file <BLOCK_FILE>...*
: File containing a list of domains to dns block, multiple can be specified

*-a, --allow-file <ALLOW_FILE>*
: File containing a list of domains to allow, even if they are in the block list

*-h, --help*
: Print help

*-V, --version*
: Print version

# EXAMPLES
  **Create the rpz.db file from multiple block lists and an allow list:**
: dns-block -dd --lists-file list_of_lists.txt own_list_of_lists.txt --block-file hosts_blocked.txt --allow-file domains.whitelisted pack --bind rpz.db

# AUTHOR
Ovidiu Ionescu
