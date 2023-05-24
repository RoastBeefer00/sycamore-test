trunk build --release --public-url sycamore-test/
rm docs/*
cp dist/* docs/
git add .
git commit -m "push to pages"
git push
