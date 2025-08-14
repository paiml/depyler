# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpQ1Ebv1/wAR_UAs72s.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
