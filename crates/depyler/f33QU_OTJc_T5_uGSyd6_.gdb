# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpoB8rYv/f33QU_OTJc_T5_uGSyd6_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
