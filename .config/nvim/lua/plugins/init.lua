return {
  {
    "stevearc/conform.nvim",
    event = 'BufWritePre',
    opts = require "configs.conform",
  },
  {
    "neovim/nvim-lspconfig",
    config = function()
      require "configs.lspconfig"
    end,
  },
  {
    'mrcjkb/rustaceanvim',
    version = '^5',
    lazy = false,
  },
  {
    "nvim-treesitter/nvim-treesitter",
     opts = {
       ensure_installed = {
      	"vim", "rust", "c", "cpp", "go", "asm"
       },
     },
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
  {
    "neovim/nvim-lspconfig",
    config = function()
      local lspconfig = require("lspconfig")
      lspconfig.asm_lsp.setup({})
      lspconfig.rust_analyzer.setup({})
    end,
  },
}
