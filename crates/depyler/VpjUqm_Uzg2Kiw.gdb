# GDB initialization script for Depyler debugging
# Source: /tmp/.tmp5Kw56t/VpjUqm_Uzg2Kiw.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
