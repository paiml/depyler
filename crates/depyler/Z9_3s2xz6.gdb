# GDB initialization script for Depyler debugging
# Source: /tmp/.tmp7Xnt9I/Z9_3s2xz6.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
