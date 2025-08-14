# GDB initialization script for Depyler debugging
# Source: /tmp/.tmp2WzOIu/of7jmgST__h7L.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
