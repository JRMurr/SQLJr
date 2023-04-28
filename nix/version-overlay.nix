self: super: {
  napi-rs-cli = super.napi-rs-cli.overrideAttrs (old: rec {

    version = "2.15.2";

    src = super.fetchurl {
      url = "https://registry.npmjs.org/@napi-rs/cli/-/cli-${version}.tgz";
      hash = "sha256-8FpvvBKkHOTqXeg9Q1YUjMF677m0YB969BHHzYdcw50=";
    };
  });
}
