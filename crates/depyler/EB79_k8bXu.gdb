# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpawczJI/EB79_k8bXu.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
