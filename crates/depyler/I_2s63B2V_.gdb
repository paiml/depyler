# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpOQED6c/I_2s63B2V_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
