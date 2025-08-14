# GDB initialization script for Depyler debugging
# Source: /tmp/.tmplGQM9a/SSV_9zD1h.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
