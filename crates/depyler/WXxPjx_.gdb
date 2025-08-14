# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpRulFbR/WXxPjx_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
