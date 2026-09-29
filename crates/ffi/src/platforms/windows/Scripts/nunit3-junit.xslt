<?xml version="1.0" encoding="UTF-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
  <xsl:output method="xml" indent="yes" />
  <xsl:template match="/test-run">
    <testsuites>
      <xsl:attribute name="tests"><xsl:value-of select="@testcasecount"/></xsl:attribute>
      <xsl:attribute name="failures"><xsl:value-of select="@failed"/></xsl:attribute>
      <xsl:attribute name="skipped"><xsl:value-of select="@skipped"/></xsl:attribute>
      <xsl:attribute name="time"><xsl:value-of select="@duration"/></xsl:attribute>
      <xsl:apply-templates select="//test-suite[@type='TestFixture']" />
    </testsuites>
  </xsl:template>
  <xsl:template match="test-suite">
    <testsuite>
      <xsl:attribute name="name"><xsl:value-of select="@fullname"/></xsl:attribute>
      <xsl:attribute name="tests"><xsl:value-of select="@testcasecount"/></xsl:attribute>
      <xsl:attribute name="failures"><xsl:value-of select="@failed"/></xsl:attribute>
      <xsl:attribute name="errors">0</xsl:attribute>
      <xsl:attribute name="skipped"><xsl:value-of select="@skipped"/></xsl:attribute>
      <xsl:attribute name="time"><xsl:value-of select="@duration"/></xsl:attribute>
      <xsl:apply-templates select=".//test-case" />
    </testsuite>
  </xsl:template>
  <xsl:template match="test-case">
    <testcase>
      <xsl:attribute name="name"><xsl:value-of select="@name"/></xsl:attribute>
      <xsl:attribute name="classname"><xsl:value-of select="@classname"/></xsl:attribute>
      <xsl:attribute name="time"><xsl:value-of select="@duration"/></xsl:attribute>
      <xsl:if test="@result='Skipped'">
        <skipped />
      </xsl:if>
      <xsl:apply-templates select="./failure" />
    </testcase>
  </xsl:template>
  <xsl:template match="failure">
    <failure>
      <xsl:attribute name="message"><xsl:value-of select="./message"/></xsl:attribute>
      <xsl:value-of select="./stack-trace" />
    </failure>
  </xsl:template>
</xsl:stylesheet>
