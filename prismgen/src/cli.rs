use clap::{
	Parser,
	Args,
};
use crate::error::Result;

#[derive(Parser, Debug)]
pub enum Cli {
	Components(CmdComponents),
}

#[derive(Args, Debug)]
pub struct CmdComponents {
	#[arg(long, default_value="github:qacow37/prismlauncher-nix/prismgen")]
	client_agent: String,

	output: String,
}

impl CmdComponents {
	pub async fn run(self) -> Result<()> {
        // An absolute mess, I dont wanna fix this
        //
        // Because it was an absolute mess
        // I tried making ascii art from it
        // I have no skills though

        use reqwest::Client;//quadrat
        /**/use std::{path::Path,fs::
        File,io::{Write,BufWriter},//
        sync::Arc,write,};use crate::
        prism::PrismIndex;use crate::
        nix;use tokio::{task::JoinSet
        ,sync::Semaphore,};#[derive//
        /**/(serde::Serialize)]struct
        Component{important:bool,uid:




        // You see that this is Kirby right? Right?!
          &'static                                  str,version:
          String,}#                               [derive(serde::
         Serialize)]                             struct Version
                    {game:Component,fabric:Option
                  <Component>,quilt:Option<Component>,
                forge:Option<Component>,neoforge:Option
             <Component>,}let opath=Path::new(&self.output);
           std::fs::      create_dir_all  (opath)?;let/*ahh*/
          default_file    ={let mut path  =opath.to_path_buf()
          ;path.push(     "default");     path.set_extension
          ("nix");File    ::create(path   /*space*/)}?;let
         mut  default_filebuf=BufWriter::new(default_file);
        default_filebuf.write_all     (b"{\n")?;let client=
        Client::builder()                .user_agent(self
        .client_agent)                    .https_only (true)
         /*prism only                      has https  API */
        .build()?;let                      index=PrismIndex
        ::new/*some*/                     (&client).await?;
          let mcpkgs=                    index.get_pkgs
            (concat!(                   "net.minecr",
             "aft")).expect(concat!("net.minecraft",
             " sh","ou","ld ","always be available"
                ));let mut tasks=JoinSet::new

            ();let                  tasks_limit=
           Arc::new(              Semaphore::new(
            64));let              mut file_index:

        // This was supposed to be the great penguin
        // Look how he looks now :(
                             usize = 0;
                          for pkg in mcpkgs{
                        use itoa::Buffer ;let
                        mut  filenamebuf
                        =     Buffer  ::new()
                        ;                 let
                       filename              =
                      filenamebuf       .format
                      (              file_index
                      );             file_index
                      +=1;                  let
                     vfabric              =index
                     .get_latest_pkg_for_mc    (
                    concat!(               "net.",
                   "fabr",                  "icmc.",
                  "fab",                       "ric",
                  "-lo",                      "ader")
                  ,pkg.get_version            ());let
                  vquilt=index.get_latest_pkg_for_mc(
                     concat!("org.quiltmc.quilt",
                         "-loa",     "der"),






                      pkg.get_version(  ));let vforge
                    =       index.get_latest_pkg_for_mc
                 (                    "net.minecraftforge"
               ,pkg.get_version())                     ;let
              vneoforge=index        .get_latest_pkg_for_mc(
             "net.neoforged"                 ,pkg.get_version
             ());      type      ComponentP5ABC    =Component
             ;type                             ComponentP4YU=
             Component;             let              version=
             Version{               game           :Component
              {important                              :true,
              uid:concat!(                         "net.mi",
              "necraft"),                           version:
               pkg.get_version                          ().
               to_string                               (),}
                ,fabric:      ((((vfabric             ))))
                 .map(|((((       p))))|       Component
                     {important               :false,
                         uid:concat!("net.fabri",







                    "cmc.fa",          "bric",
                  "-loader"),        version:p
                  .get_version()     .to_string
                     (),}),quilt:      vquilt.map
                    (|p|Component{   important:false
               ,uid:concat!("org.quilmtc.quilt-loader"),
             version:p.get_version().to_string(),}),forge:
           vforge.map(|p|{{{{{{{{{{{{ComponentP4YU{important
          :false,uid:concat!("net.minecraftforge"),version:p.
         get_version()  .to_string(),}}}}}}}}}}}}}  ),neoforge
         :vneoforge.      map(|p|ComponentP5ABC{     important
         :false,uid:      concat!("net",".neof",      "orged",
         ""),version    :p.get_version().to_string    (),}),};
         let path={let mut path=opath.to_path_buf();path.push(
         filename);path.add_extension    (concat!("nix"));path
         };let sem=tasks_limit.clone      ();tasks.spawn(async
         move{let _permit=sem.acquire   ().await.expect(concat
         !("we should be able to create a semaphore but it s",
         "mwh did not work"));let file=File::create(path)?;let
         buf=BufWriter::new(file );nix::to_writer(buf,&version
           )?;Result::<_ >::Ok(())});write!(default_filebuf,
            concat!(" ", "\"{}\""," = im", "p","ort ","./",
             "{}.nix;\n"),pkg .get_version(),filename,)?;}
               default_filebuf .write_all(b"}")?;/* wait
                till all tasks are finished */while let
                      Some(r) = tasks.join_next()
                          .await {r??;}Ok(())
	}
}

impl Cli {
	pub async fn run(self) -> Result<()> {
		match self {
			Self::Components(args) => args.run().await,
		}
	}
}
