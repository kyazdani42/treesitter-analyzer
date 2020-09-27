repositories=(
    "https://github.com/tree-sitter/tree-sitter-rust"
    "https://github.com/nvim-treesitter/tree-sitter-lua"
)

for url in ${repositories[@]}; do
    git clone --depth=1 --branch=master "$url"
    folder=$(basename "$url")
    mv $folder "../parsers/"
done

