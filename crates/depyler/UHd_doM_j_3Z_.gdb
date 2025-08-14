# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpPnrqfA/UHd_doM_j_3Z_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
