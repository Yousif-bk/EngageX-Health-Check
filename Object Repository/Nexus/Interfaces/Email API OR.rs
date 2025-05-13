<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Email API OR</name>
   <tag></tag>
   <elementGuidId>e3927828-95ee-4aff-a599-872814e96b10</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;Testcampaign1\&quot;,\n    \&quot;description\&quot;: \&quot;description\&quot;,\n    \&quot;emailBody\&quot;: \&quot;This is a test email from Beeru\&quot;,\n    \&quot;category\&quot;: \&quot;TXN\&quot;,\n    \&quot;emailFromAddr\&quot;:\&quot;test12456@eande-engagex.com\&quot;,\n    \&quot;emailToAddr\&quot;: \&quot;yusuf.babiker0@gmail.com\&quot;,\n    \&quot;subject\&quot;: \&quot;EngageX Health Check: Nexus Email API\&quot;,\n    \&quot;displayName\&quot;: \&quot;CpaasEmail1\&quot;,\n    \&quot;clientTxnId\&quot;:\&quot;7112345678901236\&quot;,\n    \&quot;sourceType\&quot; : \&quot;1\&quot;,\n    \&quot;expDuration\&quot;: \&quot;1d\&quot;\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>2d2a5806-8bca-4134-9b4c-6acd6fb3bc28</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${authToken}</value>
      <webElementGuid>f11b177c-2eb8-403a-b19a-b06c9f1da260</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://nexus.eandenterprise.com/api/v1/email/send</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>cde41323-c6a8-4bc9-a100-6c0fb49995ae</id>
      <masked>false</masked>
      <name>authToken</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
