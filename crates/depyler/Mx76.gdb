# GDB initialization script for Depyler debugging
# Source: /tmp/.tmp5FvIfw/Mx76.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
