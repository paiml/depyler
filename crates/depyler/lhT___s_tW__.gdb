# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpGh91w0/lhT___s_tW__.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
