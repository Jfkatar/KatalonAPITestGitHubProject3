<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>FahrenheitToCelsius</name>
   <tag></tag>
   <elementGuidId>d97c83a8-f55c-417f-b0fa-202984f81c5b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>SOAPAction</name>
      <type>Main</type>
      <value>https://www.w3schools.com/xml/FahrenheitToCelsius</value>
      <webElementGuid>5d87bfda-69ef-4915-bfbb-3ff1f26efca3</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
      <webElementGuid>c49c5ce4-d2c9-4b2e-ad66-983751755d90</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:x/=&quot;https://www.w3schools.com/xml/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;x/:FahrenheitToCelsius>
         &lt;x/:Fahrenheit>gero et&lt;/x/:Fahrenheit>
      &lt;/x/:FahrenheitToCelsius>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://www.w3schools.com/xml/tempconvert.asmx</soapServiceEndpoint>
   <soapServiceFunction>FahrenheitToCelsius</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <wsdlAddress>https://www.w3schools.com/xml/tempconvert.asmx?WSDL</wsdlAddress>
</WebServiceRequestEntity>
