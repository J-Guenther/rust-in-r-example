# Generated by extendr: Do not edit by hand

# nolint start

#
# This file was created with the following call:
#   .Call("wrap__make_rustInRExample_wrappers", use_symbols = TRUE, package_name = "rustInRExample")

#' @docType package
#' @usage NULL
#' @useDynLib rustInRExample, .registration = TRUE
NULL

#' Return string `"Hello from Rust!"` to R.
#' @export
hello_world <- function() .Call(wrap__hello_world)

nchar2 <- function(x) .Call(wrap__nchar2, x)


# nolint end
