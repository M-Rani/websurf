//                                                       ****                  
//                                                        ******               
// *############                                           ********,           
// *##################*                          ,*********************        
// *######################(                *******************************     
// *##########################         ************************************    
// *############################    ************************************       
//              (#################  ********************************,          
//                  *###############  ***********.         ******              
//                     ###############  *****             ****                 
//                       (##############  .              **                    
//                         ###############               ,                     
//                      ***  ###############             /###                  
//                   ********  #################          /#####               
//                *************  ######################*   /#######(           
//       ,***********************  (###################################        
// ,**************************        ####################################     
// ,************************              #################################    
// ,********************                       *#######################/       
// ,**************                                          ########           
//                                                         ######              
//                                                        ####                 
//                                                       #/   

use fastrand;
pub fn random_website() -> &'static str {
    let useless_websites: Vec<&str> = vec![
        "https://longdogechallenge.com/",
        "https://checkboxrace.com/",
        "https://onesquareminesweeper.com/",
        "http://heeeeeeeey.com/",
        "http://corndog.io/",
        "https://mondrianandme.com/",
        "https://puginarug.com",
        "https://checkboxolympics.com/",
        "https://alwaysjudgeabookbyitscover.com",
        "https://thatsthefinger.com/",
        "https://cant-not-tweet-this.com/",
        "http://eelslap.com/",
        "http://www.staggeringbeauty.com/",
        "http://burymewithmymoney.com/",
        "https://smashthewalls.com/",
        "https://jacksonpollock.org/",
        "http://endless.horse/",
        "https://www.trypap.com/",
        "http://www.republiquedesmangues.fr/",
        "http://www.movenowthinklater.com/",
        "http://www.partridgegetslucky.com/",
        "http://www.rrrgggbbb.com/",
        "http://www.koalastothemax.com/",
        "http://www.everydayim.com/",
        "http://randomcolour.com/",
        "http://cat-bounce.com/",
        "http://chrismckenzie.com/",
        "https://thezen.zone/",
        "http://hasthelargehadroncolliderdestroyedtheworldyet.com/",
        "http://ninjaflex.com/",
        "http://ihasabucket.com/",
        "http://corndogoncorndog.com/",
        "http://www.hackertyper.com/",
        "https://pointerpointer.com",
        "http://imaninja.com/",
        "http://drawing.garden/",
        "http://www.ismycomputeron.com/",
        "http://www.nullingthevoid.com/",
        "http://www.muchbetterthanthis.com/",
        "http://www.yesnoif.com/",
        "http://lacquerlacquer.com",
        "http://potatoortomato.com/",
        "http://iamawesome.com/",
        "https://strobe.cool/",
        "http://thisisnotajumpscare.com/",
        "http://doughnutkitten.com/",
        "http://crouton.net/",
        "http://corgiorgy.com/",
        "http://www.wutdafuk.com/",
        "http://unicodesnowmanforyou.com/",
        "http://chillestmonkey.com/",
        "http://scroll-o-meter.club/",
        "http://www.crossdivisions.com/",
        "http://tencents.info/",
        "https://boringboringboring.com/",
        "http://www.patience-is-a-virtue.org/",
        "http://pixelsfighting.com/",
        "http://isitwhite.com/",
        "https://existentialcrisis.com/",
        "http://onemillionlols.com/",
        "http://www.omfgdogs.com/",
        "http://oct82.com/",
        "http://chihuahuaspin.com/",
        "https://popcat.click/",
        "http://www.blankwindows.com/",
        "http://tunnelsnakes.com/",
        "http://www.trashloop.com/",
        "http://www.ascii-middle-finger.com/",
        "http://spaceis.cool/",
        "http://www.donothingfor2minutes.com/",
        "http://buildshruggie.com/",
        "http://buzzybuzz.biz/",
        "http://yeahlemons.com/",
        "http://wowenwilsonquiz.com",
        "https://thepigeon.org/",
        "http://notdayoftheweek.com/",
        "http://www.amialright.com/",
        "http://nooooooooooooooo.com/",
        "https://greatbignothing.com/",
        "https://zoomquilt.org/",
        "https://dadlaughbutton.com/",
        "https://www.bouncingdvdlogo.com/",
        "https://remoji.com/",
        "http://papertoilet.com/",
        "https://loopedforinfinity.com/",
        "https://findtheinvisiblecow.com/",
        "https://wiby.me/surprise/",
        "https://www.bestuselesswebsites.com/go/",
        "http://alien-ufo-research.com/reptilians/",
        "https://www.flightradar24.com/",
        "http://nooooooooooooooo.com/",
        "http://secrettechnology.com/",
        "http://www.patience-is-a-virtue.org/",
        "https://isitchristmas.com/",
        "http://omfgdogs.com/",
        "http://zombo.com/",
        "http://www.rainymood.com/",
        "http://www.fallingfalling.com/",
        "http://corgiorgy.com/",
        "http://www.pointerpointer.com/",
        "http://eelslap.com/",
        "http://zoomquilt.org/",
        "http://beesbeesbees.com/",
        "http://www.thepictureofeverything.com/",
        "http://doughnutkitten.com/",
        "https://www.futureme.org/",
        "http://www.newyearexercise.com/",
        "https://web.archive.org/web/20180201230730/http://www.dragomirsdiary.com/2011/08/hello-diary.html",
        "http://5secondfilms.com/",
        "http://alpha61.com/primenumbershittingbear/",
        "http://www.myhousematesdiary.com/",
        "http://www.cachemonet.com/",
        "http://clickingbad.nullism.com/",
        "http://www.gizoogle.net/",
        "https://ncase.me/trust/",
        "http://www.clicktoremove.com/",
        "http://vectorpark.com/head/",
        "https://www.purristan.com/",
        "https://www.airfrais.eu/us/index.html",
        "http://www.nobodyhere.com/justme/me.here",
        "http://www.superbad.com/",
        "http://therevolvinginternet.com/",
        "http://get.your-d.tk/",
        "http://www.miserablebastard.com/",
        "http://www.freechocolate.com/",
        "https://morelessgame.com/",
        "http://sheepfilms.co.uk/category/shortfilms/",
        "http://strangehorizons.com/non-fiction/articles/installing-linux-on-a-dead-badger-users-notes/",
        "http://perfectlytimedphotos.com/popular/perfectly-timed-photo",
        "http://jpriest.com/gamez/pinguxtreme.swf",
        "http://www.bradthegame.com/",
        "http://theworstthingsforsale.com/",
        "https://s3.mirror.co.uk/click-the-colour-and-not-the-word/index.html",
        "http://theoatmeal.com/comics/dog_paradox",
        "http://www.catbirdseat.org/catbirdseat/bingo.html",
        "http://unomoralez.com/",
        "http://www.fincher.org/Misc/Pennies/BigTower.shtml",
        "http://www.cleverbot.com/",
        "http://www.hat.net/abs/noclick/index.html",
        "http://www.staggeringbeauty.com/",
        "http://burymewithmymoney.com/",
        "http://just-shower-thoughts.tumblr.com/",
        "http://www.trypap.com/",
        "http://www.republiquedesmangues.fr/",
        "http://www.koalastothemax.com/",
        "http://www.carrotmuseum.co.uk/carrotcolours.html",
        "http://grandpanoclothes.com/",
        "http://www.everydayim.com/",
        "http://hasthelargehadroncolliderdestroyedtheworldyet.com/",
        "http://ninjaflex.com/",
        "http://chrismckenzie.com/",
        "http://corndogoncorndog.com/",
        "http://gameaboutsquares.com/",
        "https://ncase.me/trust/",
        "http://salmonofcapistrano.com/",
        "http://www.wutdafuk.com/",
        "http://www.ouaismaisbon.ch/",
        "http://unicodesnowmanforyou.com/",
        "http://www.psyhigh.com/",
        "http://www.internetisshit.org/",
        "http://www.fmylife.com/",
        "http://www.realultimatepower.net/index4.htm",
        "http://rulesoftheinternet.com/index.php?title=Main_Page",
        "http://www.markdalderup.com/daylight-of-darkness/the-start-of-the-world/",
        "http://presidentialpickuplines.tumblr.com/",
        "http://sarina-brewer.com/",
        "http://chickswithstevebuscemeyes.tumblr.com/",
        "http://thingsididlastnight.com/",
        "http://www.heptune.com/farts.html",
        "http://inventorspot.com/articles/body_bread_13546",
        "http://memebase.cheezburger.com/pictureisunrelated",
        "http://alien-ufo-research.com/news/2013/proof-of-time-travelers.php",
        "http://pixelsfighting.com/",
        "http://hardcoreprawnlawn.com/",
        "http://www.slightlyinteresting.com/lavalamp/lava.asp",
        "http://thefo.nz/",
        "http://faceofdisapproval.com/",
        "http://semanticresponsiveillustration.com/",
        "http://dogs.are.the.most.moe/",
        "https://web.archive.org/web/20190512120348/https://oct82.com/",
        "http://www.drawastickman.com",
        "https://trek.nasa.gov/mars/index.html",
        "http://ihumans.tumblr.com/",
        "http://whos.agood.dog/lesser.dog/",
        "http://chatwithhodor.com/",
        "https://web.archive.org/web/20170514160422/http://manbabies.com/",
        "http://www.rinkworks.com/bookaminute/classics.shtml",
        "http://www.ijustwantyourmoney.com/",
        "http://www.onemilliongiraffes.com/",
        "http://www.lkozma.net/wpv/index.html",
        "http://www.fieggen.com/shoelace/index.htm",
        "http://playing.hypernom.com/monkeys",
        "http://www.sam-i-am.com/play/5k/expletives/index.html",
        "http://beaarthurmountainspizza.tumblr.com/",
        "http://www.stopabductions.com/",
        "https://swatblog.rtgp.xyz/",
        "http://kimjongillookingatthings.tumblr.com/",
        "http://garfieldminusgarfield.net/",
        "http://dogeweather.com/",
        "http://www.hen2hen.com/",
        "https://web.archive.org/web/20160725231633/http://celebritiesaskids.net/",
        "http://codepen.io/ge1doot/full/WbWQOP/",
        "http://icanhas.cheezburger.com/",
        "http://www.engrish.com/",
        "http://www.astrohamster.com/",
        "http://stuffonmycat.com/",
        "http://www.bbc.com/future/bespoke/story/20150306-journey-to-the-centre-of-earth/index.html",
        "http://www.cowsgomoo.co.uk/",
        "http://www.woot.co.uk/",
        "https://bouncingdvdlogo.com/",
        "http://orteil.dashnet.org/cookieclicker/",
        "http://themostseconds.com/",
        "http://www.midwaymeetup.com/",
        "https://www.youtube.com/watch?v=9C_HReR_McQ&feature=emb_logo",
        "http://www.handspeak.com/word/",
        "http://topdocumentaryfilms.com/",
        "http://www.madsci.org/cgi-bin/lynn/jardin/SCG",
        "http://trumpdonald.org/",
        "https://drawception.com",
        "http://badkidsjokes.tumblr.com/",
        "http://internet-map.net/",
        "https://youtu.be/9C_HReR_McQ",
        "http://hyperboleandahalf.blogspot.co.uk/",
        "http://slither.io/",
        "http://www.youareinaforest.com/",
        "http://remoji.com/",
        "http://shiiiit.com/",
        "http://www.howmanypeopleareinspacerightnow.com/",
        "http://hmpg.net/",
        "http://conceptlab.com/simulator/morning/clock800.html",
        "https://www.kamogo.com/9",
        "http://www.therestartpage.com/",
        "http://www.stinkymeat.net/",
        "http://www.eviloverlord.com/lists/overlord.html",
        "http://www.windows93.net/",
        "http://thefuckingtime.com/",
        "http://make-everything-ok.com/",
        "http://what-if.xkcd.com/8/",
        "http://www.danielyeow.com/2011/drawing-molecules/",
        "http://inventorspot.com/articles/ten_bizarre_japanese_soft_drinks_5225",
        "http://www.wikihow.com/Recycle-Your-Socks",
        "https://www.dctech.com/physics/notes/0005.php",
        "http://www.pintprice.com/",
        "http://www.barefooters.org/",
        "http://www.weirdconverter.com/",
        "http://www.theflatearthsociety.org/cms/",
        "http://scienceblogs.com/goodmath/2006/10/12/who-needs-a-calculator-multipl/",
        "http://www.coincalc.com/",
        "http://festivusweb.com/",
        "http://www.appropedia.org/Solar_Charged_Lawnmower",
        "http://www.genderanalyzer.com/",
        "http://www.muppetlabs.com/~breadbox/txt/al.html",
        "http://fliptitle.com/",
        "http://www.worldbeardchampionships.com/photos/",
        "http://www.fishcam.com/",
        "http://www.ehow.com/how_4594591_suck-egg-bottle.html",
        "https://www.youtube.com/watch?v=jU9USxJ9vPs",
        "http://www.venganza.org/",
        "http://www.humanforsale.com/",
        "http://www.willitblend.com/",
        "http://www.icbe.org/",
        "http://www.museumofconceptualart.com/accomplished/index.html",
        "https://scatter.wordpress.com/2010/05/30/the-shortest-possible-game-of-monopoly-21-seconds/",
        "http://www.thesmokinggun.com/time-waster",
        "https://www.youtube.com/askaninja",
        "http://www.godecookery.com/mythical/mythical.htm",
        "http://www.sandalandsoxer.co.uk/home.htm",
        "http://www.lileks.com/oldads/index.html",
        "http://australianmuseum.net.au/death-the-last-taboo",
        "http://www.shadyurl.com/",
        "http://memebase.cheezburger.com/thisisphotobomb",
        "https://www.gutenberg.org/",
        "http://ipetcompanion.com",
        "http://findtheinvisiblecow.com/",
        "https://web.archive.org/web/20190618062455/http://www.iloveyoulikeafatladylovesapples.com/",
        "http://www.thepointless.com/reddot",
        "http://www.rrrather.com/",
        "http://virtualpiano.net/",
        "http://drunkenme.com/movie-drinking-games/",
        "http://findagrave.com/",
        "https://onetinyhand.com/",
        "http://zoomquilt.org/",
        "http://cachemonet.com/",
        "http://www.plspetdoge.com/",
        "http://salmonofcapistrano.com/",
        "http://www.firstmenonthemoon.com/",
        "http://codepen.io/akm2/full/rHIsa",
        "https://www.housecreep.com/",
        "http://you.regettingold.com/",
        "http://hotdogcat.com/",
        "http://instantrimshot.com/",
        "http://www.anothersadtrombone.com/",
        "https://web.archive.org/web/20180211011023/http://cutecatnames.com/",
        "http://www.instantwhoop.com",
        "http://drunkenme.com/movie-drinking-games/",
        "https://findtheinvisiblecow.com/",
        "https://www.mapcrunch.com/",
        "https://hackertyper.com/",
        "http://papertoilet.com/",
        "https://parade.com/1116816/marynliles/fun-websites/",
        "https://screamintothevoid.com/",
        "http://www.shadyurl.com/",
        "https://www.shutupandtakemymoney.com/",
        "https://play2048.co/",
        "http://www.drivemeinsane.com/",
        "http://orteil.dashnet.org/cookieclicker/",
        "http://www.foddy.net/",
        "https://quickdraw.withgoogle.com/",
        "https://www.addictivetips.com/",
        "http://burymewithmymoney.com/",
        "https://coronavirus-ninja.com/",
        "https://therandombutton.github.io/random.html",
        "https://15.ai/",
        "http://copy.sh/v86/", 
        "https://cock.li/",
        "https://yopmail.com/en/",
        "https://www.guerrillamail.com/",
        "https://codewithrockstar.com/online",
        "https://www.internetlivestats.com/"
    ];
    let useless_websites_length: i32 = (useless_websites.len() - 1).try_into().unwrap();
    let random_number = fastrand::i32(0..useless_websites_length);

    return useless_websites[random_number as usize];
}
