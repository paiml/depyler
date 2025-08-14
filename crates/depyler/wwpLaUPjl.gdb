# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpFuQ2gM/wwpLaUPjl.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
