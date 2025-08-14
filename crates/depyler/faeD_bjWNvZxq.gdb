# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpjf9pcg/faeD_bjWNvZxq.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
