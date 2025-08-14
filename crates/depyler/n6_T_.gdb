# GDB initialization script for Depyler debugging
# Source: /tmp/.tmp6hfBNi/n6_T_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
