return {
	{
		"mrcjkb/rustaceanvim",
		version = "^5",
		lazy = false,
	},
	{
		"nvim-treesitter/nvim-treesitter",
		opts = {
			ensure_installed = {
				"vim",
				"rust",
				"c",
				"cpp",
				"go",
				"asm",
			},
		},
	},
	{
		"Who5673/who5673-nasm",
		dependencies = {
			"L3MON4D3/LuaSnip",
			"hrsh7th/nvim-cmp",
		},
		ft = "nasm",
		lazy = true,
	},
	{
		"williamboman/mason-lspconfig.nvim",
		dependencies = { "williamboman/mason.nvim", "neovim/nvim-lspconfig" },
		config = function()
			require("mason-lspconfig").setup({
				ensure_installed = {
					"asm_lsp",
					"rust_analyzer",
				},
				automatic_installation = true,
			})
		end,
	},
}
