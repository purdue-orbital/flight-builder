line=$(cargo +nightly llvm-cov | tail -1 | awk '{print $3 $6 $9}');

if [ "$line" = "1814" ]; then
	exit 0;
else
	exit 1;
fi
