<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="3.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform"
                xmlns:atom="http://www.w3.org/2005/Atom" xmlns:dc="http://purl.org/dc/elements/1.1/"
                xmlns:itunes="http://www.itunes.com/dtds/podcast-1.0.dtd">
  <xsl:output method="html" version="1.0" encoding="UTF-8" indent="yes"/>
  <xsl:template match="/">
    <html xmlns="http://www.w3.org/1999/xhtml" lang="en">
      <head>
        <title><xsl:value-of select="/rss/channel/title"/> RSS feed</title>
        <meta http-equiv="Content-Type" content="text/html; charset=utf-8"/>
        <meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1"/>
        <style type="text/css">*,:before,:after{box-sizing:border-box}*{margin:0}html,body{height:100%}body{-webkit-font-smoothing:antialiased;line-height:calc(1rem + .75em)}img,picture,video,canvas,svg{max-width:100%;display:block}input,button,textarea,select{font:inherit}p,h1,h2,h3,h4,h5,h6{overflow-wrap:break-word}#__root{isolation:isolate;max-width:40rem;margin:auto;padding:1.5rem}body{font-family:Segoe UI Variable,Segoe UI,-apple-system,BlinkMacSystemFont,"ui-sans-serif",sans-serif}header{padding-top:2rem}@media (min-width:768px){header{padding-top:5rem}}h1,h2{border-bottom:1px solid #eaecef;padding-bottom:.3em;font-size:1.5em}h1{margin:1rem 0;font-size:2.25em}h2{margin:1.75em 0 .5em;font-size:1.75em}h3{margin:1.5em 0 0;font-size:1.375em}a{color:#7a3500}a:hover{text-decoration:none}.visit{font-size:.9em;font-weight:500}</style>
      </head>
      <body>
        <div id="__root">
          <header>
            <h1><xsl:value-of select="/rss/channel/title"/></h1>
            <p><xsl:value-of select="/rss/channel/description"/></p>
            <a class="visit" target="_blank">
              <xsl:attribute name="href">
                <xsl:value-of select="/rss/channel/link"/>
              </xsl:attribute> Visit Website &#x2192;
            </a>
          </header>
          <main>
            <h2>Recent posts</h2>
            <xsl:for-each select="/rss/channel/item">
              <aside>
                <h3>
                  <a target="_blank">
                    <xsl:attribute name="href">
                      <xsl:value-of select="link"/>
                    </xsl:attribute>
                    <xsl:value-of select="title"/>
                  </a>
                </h3>
                <small>
                  Published: <xsl:value-of select="pubDate" />
                </small>
              </aside>
            </xsl:for-each>
          </main>
        </div>
      </body>
    </html>
  </xsl:template>
</xsl:stylesheet>
