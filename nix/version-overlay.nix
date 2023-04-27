self: super: {
  sl = super.napi-rs-cli.overrideAttrs (old: rec {

    version = "2.15.2";

    src = super.fetchurl {
      url = "https://registry.npmjs.org/@napi-rs/cli/-/cli-${version}.tgz";
      hash = "";
    };
  });
}
