# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpG2IkRO/DfWa6c_qXRInkYX5Gl5I.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
