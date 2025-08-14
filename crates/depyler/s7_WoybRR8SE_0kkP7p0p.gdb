# GDB initialization script for Depyler debugging
# Source: /tmp/.tmppCRfoO/s7_WoybRR8SE_0kkP7p0p.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
