RUST=cargo rustc
FLAGS=link-arg=-nostartfiles

make:
	$(RUST) -- -C $(FLAGS)