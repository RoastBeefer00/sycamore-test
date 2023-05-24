trunk build --release --public-url sycamore-test/
rm docs/*
cp dist/* docs/
dopush "push to pages"
