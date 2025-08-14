# GDB initialization script for Depyler debugging
# Source: /tmp/.tmp6BdMDg/k_9l_yIWSYbG.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
