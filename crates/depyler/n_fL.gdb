# GDB initialization script for Depyler debugging
# Source: /tmp/.tmprpBkKI/n_fL.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
