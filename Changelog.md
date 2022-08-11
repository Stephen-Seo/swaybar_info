# Changelog

## Upcoming Changes

## 0.1.2

Fix README.md (erroneously used `--date-format` instead of `--time-format`).

## 0.1.1

Implement setting the time format string with the `--time-format=<time format string>` arg.

## 0.1.0

Implementation of info output for use by swaybar.

By default, displays the date, load-avg, and memory usage.

Network stats can be shown with the `--netdev=<device>` arg.

Arbitrary output from a command can be shown with the `--regex-cmd=<cmd>[SPLIT]<args...>[SPLIT]<regex>` arg.

The update interval in seconds can be set with the `--interval-sec=<seconds>` arg.

Battery stats display can be enabled with the `--acpi-builtin` arg.
