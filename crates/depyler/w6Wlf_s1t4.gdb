# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpoHOSHo/w6Wlf_s1t4.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
