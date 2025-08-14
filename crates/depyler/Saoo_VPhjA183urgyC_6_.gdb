# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpRVrrN5/Saoo_VPhjA183urgyC_6_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
