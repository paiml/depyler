# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpr7a3Sh/eZd0hbaQ43b_OpcKN_H.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
