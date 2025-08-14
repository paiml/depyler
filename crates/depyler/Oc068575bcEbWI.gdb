# GDB initialization script for Depyler debugging
# Source: /tmp/.tmp4Lk5eB/Oc068575bcEbWI.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
