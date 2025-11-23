/**
 * @file Describes into a single file how lan items should be added to DNS and DHCP
 * @author Ovidiu Ionescu <ovidiu@ionescu.net>
 * @license AGPL3
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check


module.exports = grammar({
  name: "orgncf",
  extras: ($) => [
    /\s/,
    $.comment,
  ],
  rules: {
    source_file: ($) =>
      seq(
        repeat(choice($.ip_config_line, $._extra_config_line)),
        $.dns_prefix_section,
        $.dns_suffix_section,
        $.dhcp_prefix_section,
      ),
    ip_config_line: ($) => choice($._rule),
    _extra_config_line: ($) =>
      choice(
        $.domain, 
        $.dns_file_name, 
        $.reverse_dns_file_name, 
        $.dhcp_file_name
      ),
    domain:                $ => seq($.domain_keyword, $.hostname, "\n"),
    dns_file_name:         $ => seq($.dns_file_name_keyword, $.filename, "\n"),
    reverse_dns_file_name: $ => seq($.reverse_dns_file_name_keyword, $.filename, "\n"),
    dhcp_file_name:        $ => seq($.dhcp_file_name_keyword, $.filename, "\n"),
    
    domain_keyword: ($) => /domain/,
    dns_file_name_keyword: ($) => /dns_file_name/,
    reverse_dns_file_name_keyword: ($) => /reverse_dns_file_name/,
    dhcp_file_name_keyword: ($) => /dhcp_file_name/,

    _rule: ($) =>
      seq(
        optional($.mac_address),
        $.ip_address,
        repeat1($.hostname),
        "\n",
      ),
    dns_prefix_section: ($) => seq(
      $.dns_prefix_keyword,
      $.string,
    ),
    dns_suffix_section: ($) => seq(
      $.dns_suffix_keyword,
      $.string,
    ),
    dhcp_prefix_section: ($) => seq(
      $.dhcp_prefix_keyword,
      $.string,
    ),
    mac_address: ($) => /([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})/,
    ip_address: ($) => /(\d{1,3}\.){3}\d{1,3}/,
    hostname: ($) => /([a-zA-Z0-9.-]+)|@/,
    comment: ($) => /;[^\n]*/,
    filename: ($) => /[a-zA-Z0-9._-]+/,
    dns_prefix_keyword: ($) => /DNS_PREFIX/,
    dns_suffix_keyword: ($) => /DNS_SUFFIX/,
    dhcp_prefix_keyword: ($) => /DHCP_PREFIX/,
    string: $ => seq(
      $.string_delimiter,
      $.string_content,
      $.string_delimiter,
    ),
    string_content: $ => token(repeat(choice(/[^"]/, /"[^"]/, /\n/))),
    string_delimiter: $ => '"""',
  },
});
